<#
.SYNOPSIS
    XML Generator for formalized documents (Stage 1.1) from primary.md.
.DESCRIPTION
    Uses a strict, schema-driven mapping to convert parsed tables from primary.md
    into windows-1251 encoded XML files required by Russian customs.
#>

param(
    [Parameter(Mandatory=$true)]
    [string]$CaseName,   # E.g. "МоскитнаяСетка"

    [Parameter(Mandatory=$true)]
    [string]$HobotRoot   # Path to the project root
)

# Stop on critical errors
$ErrorActionPreference = 'Stop'

# Configure paths
$inPath  = Join-Path (Join-Path (Join-Path $HobotRoot 'alta\result') $CaseName) 'primary.md'
$outDir  = Join-Path (Join-Path (Join-Path $HobotRoot 'alta\result') $CaseName) 'alta_import'

if (-not (Test-Path -LiteralPath $inPath)) {
    throw "ERROR: Input file not found: $inPath"
}

# Ensure output folder exists
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

# --------------------------------------------------------------
# ERROR COLLECTOR
# --------------------------------------------------------------
$Errors = New-Object System.Collections.Generic.List[string]
function Add-Error([string]$msg) { $Errors.Add($msg) | Out-Null }

# --------------------------------------------------------------
# STEP 1: PARSING
# --------------------------------------------------------------
$lines = Get-Content -LiteralPath $inPath -Encoding UTF8

$map = @{}      # карта полей
$docs = @{}     # карта документов
$currentDoc = $null
$currentArrayPrefix = $null

<#
.SYNOPSIS
    Splits a Markdown table row into trimmed cell values.
#>
function Split-Row([string]$row) {
    $r = $row.Trim()
    if (-not $r.StartsWith('|')) { return @() }
    $parts = $r.Split('|')
    $cells = @()
    for ($i = 1; $i -lt $parts.Length - 1; $i++) {
        $cells += $parts[$i].Trim()
    }
    return $cells
}

# Loop through primary.md lines
$lineNum = 0
foreach ($ln in $lines) {
    $lineNum++

    # Detect the beginning of a new document boundary
    if ($ln -match '###\s+`document`:\s+(.+)') {
        $currentDoc = @{ Name = $Matches[1].Trim(); Prefix = ''; Root = ''; Code = '' }
        $currentArrayPrefix = $null
        continue
    }

    # Extract current document metadata (Prefix and Root)
    if ($null -ne $currentDoc) {
        if ($ln -match '-\s+`uqi_prefix`:\s+(.+)') {

            $prefix = $Matches[1].Trim()
            $currentDoc.Prefix = $prefix

            # Добавляем в список документов
            if ($currentDoc.Prefix.StartsWith("formalized."))
            {
                $docs[$prefix] = $currentDoc
            }
            continue
        }
        if ($ln -match '-\s+`xml_target_root`:\s+(.+)') {
            $currentDoc.Root = $Matches[1].Trim()
            continue
        }
    }

    # Detect the beginning of an array boundary and extract the array name
    if ($ln -match 'Элемент массива:\s+([\w]+)') {
        $currentArrayName = $Matches[1]
        $currentArrayPrefix = $null # Reset prefix until we parse the official index marker
        continue
    }

    # Extract the official element index and construct the unique array prefix.
    # ASSERTION: If we find '_element_num' but have no array name, throw an error.
    if ($ln -match '_element_num:\s*(\d+)') {
        if ([string]::IsNullOrEmpty($currentArrayName)) {
            throw "ERROR: [Line $lineNum] Found '_element_num' marker but array name is undefined! Line: $ln"
        }
        $arrIdx = $Matches[1] # The official index (e.g., "1")
        $currentArrayPrefix = "$($currentDoc.Prefix).$currentArrayName`_$arrIdx"
        continue
    }

    # Reset array context when we hit the explicit end-of-array marker
    if ($ln -match '_item_audit:\s*\d+') {
        $currentArrayPrefix = $null
        $currentArrayName   = $null # Clear the array name as well
    }

    # Check if the current line is a valid Markdown table data row (starts with '|' followed by 2 digits)
    if ($ln -match '^\|\s*\d{2}\s*\|') {

        # 1. Проверяем, находимся ли мы вообще внутри какого-либо документа
        if ($null -eq $currentDoc) {
            throw "ERROR: [Line $lineNum] Found table row outside of any document context! Line text: $ln"
        }

        # 2. Проверяем, определен ли префикс документа (uqi_prefix)
        if ([string]::IsNullOrEmpty($currentDoc.Prefix)) {
            throw "ERROR: [Line $lineNum] Document '$($currentDoc.Name)' is missing 'uqi_prefix' metadata! Line text: $ln"
        }

        # 3. Пропускаем обработку таблиц для неформализуемых документов и мастер-данных
        if (-not $currentDoc.Prefix.StartsWith("formalized.")) {
            continue
        }

        # 4. Для оставшихся (формализуемых) документов проверяем наличие xml_target_root
        if ([string]::IsNullOrEmpty($currentDoc.Root)) {
            throw "ERROR: [Line $lineNum] Formalizable document '$($currentDoc.Name)' is missing 'xml_target_root' metadata! Line text: $ln"
        }

        # Split the Markdown row by the pipe '|' character to extract individual cells
        $cells = Split-Row $ln

        # We expect at least 4 columns: [0]num, [1]field, [2]value, [3]status
        if ($cells.Count -ge 4)
        {
            $num    = $cells[0]          # The 1-based line index (e.g., "14" or "24")
            $field  = $cells[1]          # The field name (e.g., "PostalCode")
            $value  = $cells[2]          # The actual data value
            $status = $cells[3].ToLower() # The status (e.g., "cd", "co", "pending")

            # Determine the base prefix for the Unique Query Identifier (UQI).
            # If we are inside an array element, use the array prefix. Otherwise, use the document prefix.
            $uqiBase = ""
            if (-not [string]::IsNullOrEmpty($currentArrayPrefix))
            {
                $uqiBase = $currentArrayPrefix
            }
            elseif ($null -ne $currentDoc)
            {
                $uqiBase = $currentDoc.Prefix
            }

            # If we have a valid base prefix, construct the UQI keys
            if (-not [string]::IsNullOrEmpty($uqiBase))
            {
                # Construct the name-based UQI (e.g., "formalized.invoice_1.CurrencyCode")
                $uqiField = if ($field.StartsWith($uqiBase))
                {
                    $field
                }
                else
                {
                    "$uqiBase.$field"
                }

                # Construct the index-based UQI (e.g., "formalized.service_invoice_1.14")
                $uqiNum = "$uqiBase.$num"

                # Store the parsed data object using BOTH keys.
                # This dual-key approach prevents identical field names (like PostalCode) from overwriting each other.
                $map[$uqiField] = [PSCustomObject]@{ Value = $value; Status = $status }
                $map[$uqiNum]   = [PSCustomObject]@{ Value = $value; Status = $status }
            }

            # If this field is the document code, save it to the current document object.
            # We will use this code (e.g., "03011") to generate the output XML filename later.
            if ($field -eq 'doc_code' -or $field -eq 'DocumentCode')
            {
                if ($null -ne $currentDoc)
                {
                    $currentDoc.Code = $value
                }
            }
        }
    }
}

# --------------------------------------------------------------
# GETTERS & VALIDATION
# --------------------------------------------------------------
<#
.SYNOPSIS
    Checks if a key exists in the parsed map.
#>
function HasKey([string]$k) {
    return $map.ContainsKey($k)
}

<#
.SYNOPSIS
    Safely resolves a field value with strict status checking.
.DESCRIPTION
    If a field has a 'pending' status, registers a validation error and returns an empty string.
    Converts date formats from DD.MM.YYYY to standard YYYY-MM-DD.
#>
function V([string]$k, [string]$context='') {
    $ctxStr = $k
    if (-not [string]::IsNullOrWhiteSpace($context)) { $ctxStr = "$k (for tag $context)" }
    
    if (-not $map.ContainsKey($k)) {
        Add-Error "Missing required field: $ctxStr"
        return ''
    }
    
    $fieldData = $map[$k]
    if ($fieldData.Status -eq 'pending') {
        Add-Error "Field is not filled (pending status): $ctxStr"
        return ''
    }
    
    $val = $fieldData.Value
    # Normalize dates
    if ($val -match '^(\d{2})\.(\d{2})\.(\d{4})$') {
        $val = "$($Matches[3])-$($Matches[2])-$($Matches[1])"
    }
    return $val
}

<#
.SYNOPSIS
    Escapes XML entities and replaces characters incompatible with windows-1251.
.DESCRIPTION
    Converts superscript squared/cubed symbols (², ³) to ASCII digits (2, 3).
    Normalizes smart quotes and dashes.
#>
function Get-SafeXml([string]$val) {
    if ([string]::IsNullOrEmpty($val)) { return '' }
    
    $s = $val
    # Replace symbols incompatible with windows-1251 code page
    $s = $s -replace '²', '2'
    $s = $s -replace '³', '3'
    $s = $s -replace '[–—]', '-'
    $s = $s -replace '[“”]', '"'
    $s = $s -replace '[‘’]', "'"
    
    return [System.Security.SecurityElement]::Escape($s)
}

<#
.SYNOPSIS
    Writes a single XML tag.
.DESCRIPTION
    If the value starts with 'link:', writes an empty tag and adds
    a RESOLVE_LINK comment for the AI agent to replace later.
#>
function WriteEl([System.Xml.XmlWriter]$w, [string]$name, [string]$value, [string]$uqi='') {
    $w.WriteStartElement($name)
    if ($value -match '^link:') {
        $w.WriteRaw('')
        $w.WriteComment(" RESOLVE_LINK: $value ")
    } else {
        $w.WriteRaw((Get-SafeXml $value))
    }
    $w.WriteEndElement()
}

<#
.SYNOPSIS
    Writes nested tags using dot notation (A.B.C -> <A><B><C>value</C></B></A>).
#>
function WriteNestedEl([System.Xml.XmlWriter]$w, [string]$path, [string]$value, [string]$uqi='') {
    $parts = $path.Split('.')
    for ($i=0; $i -lt $parts.Length - 1; $i++) { $w.WriteStartElement($parts[$i]) }
    WriteEl $w $parts[-1] $value $uqi
    for ($i=$parts.Length - 2; $i -ge 0; $i--) { $w.WriteEndElement() }
}

<#
.SYNOPSIS
    Resolves UQI value and writes the XML element.
#>
function WV([System.Xml.XmlWriter]$w, [string]$tagName, [string]$uqi) {
    $val = V $uqi $tagName
    if ($tagName -match '\.') {
        WriteNestedEl $w $tagName $val $uqi
    } else {
        WriteEl $w $tagName $val $uqi
    }
}

# --------------------------------------------------------------
# XML GENERATION
# --------------------------------------------------------------
# Configure windows-1251 encoding with safe Replacement Fallback for unmappable characters
$encoderFallback = New-Object System.Text.EncoderReplacementFallback '?'
$decoderFallback = New-Object System.Text.DecoderReplacementFallback '?'
$enc = [System.Text.Encoding]::GetEncoding('windows-1251', $encoderFallback, $decoderFallback)

foreach ($doc in $docs.Values) {
    # Skip non-formalized documents (where xml_target_root is empty)
    if ([string]::IsNullOrWhiteSpace($doc.Root)) { continue }

    # Output file name setup
    $safeName = $doc.Name -replace '[^\w\s-]', ''
    $n = 1
    if ($doc.Prefix -match '_(\d+)$') {
        $n = $Matches[1]
    }
    $fileName = if (-not [string]::IsNullOrWhiteSpace($doc.Code)) { "${safeName}_${n}_$($doc.Code).xml" }
        else { "${safeName}_${n}.xml" }
    $outPath = Join-Path $outDir $fileName

    $settings = New-Object System.Xml.XmlWriterSettings
    $settings.Encoding = $enc
    $settings.Indent = $true
    $settings.OmitXmlDeclaration = $false

    $w = [System.Xml.XmlWriter]::Create($outPath, $settings)
    $w.WriteStartDocument()
    $w.WriteStartElement($doc.Root)

    $p = $doc.Prefix

    # Explicit structural switches mapping to the Custom schemas
    switch ($doc.Root) {

        "AltaE2I" {
            WriteEl $w 'DocumentCode' '04021'
            WV $w 'CurrencyRate'                           "$p.CurrencyRate"
            WV $w 'CurrencyCode'                           "$p.CurrencyCode"
            WV $w 'PlacesQuantity'                         "$p.PlacesQuantity"
            WV $w 'PlacesDescription'                      "$p.PlacesDescription"
            WV $w 'GrossWeightQuantity'                    "$p.GrossWeightQuantity"
            WV $w 'NetWeightQuantity'                      "$p.NetWeightQuantity"
            WV $w 'GCost'                                  "$p.GCost"
            WV $w 'TotalCost'                              "$p.TotalCost"
            WV $w 'DeliveryTerms_DeliveryPlace'            "$p.DeliveryTerms_DeliveryPlace"
            WV $w 'DeliveryTerms_DeliveryTermsNumericCode' "$p.DeliveryTerms_DeliveryTermsNumericCode"
            WV $w 'DeliveryTerms_DeliveryTermsStringCode'  "$p.DeliveryTerms_DeliveryTermsStringCode"
            WV $w 'DeliveryTerms_DispatchCountryCode'      "$p.DeliveryTerms_DispatchCountryCode"
            WV $w 'DeliveryTerms_TradingCountryCode'       "$p.DeliveryTerms_TradingCountryCode"
            WV $w 'DeliveryTerms_DestinationCountryCode'   "$p.DeliveryTerms_DestinationCountryCode"
            WV $w 'Registration_PrDocumentName'            "$p.Registration_PrDocumentName"
            WV $w 'Registration_PrDocumentNumber'          "$p.Registration_PrDocumentNumber"
            WV $w 'Registration_PrDocumentDate'            "$p.Registration_PrDocumentDate"
            WV $w 'Contract_PrDocumentNumber'              "$p.Contract_PrDocumentNumber"
            WV $w 'Contract_PrDocumentDate'                "$p.Contract_PrDocumentDate"
            WV $w 'Buyer_CompanyID'                        "$p.Buyer_CompanyID"
            WV $w 'Buyer_KPPCode'                          "$p.Buyer_KPPCode"
            WV $w 'Buyer_Name'                             "$p.Buyer_Name"
            WV $w 'Buyer_PostalAddress_PostalCode'         "$p.Buyer_PostalAddress_PostalCode"
            WV $w 'Buyer_PostalAddress_CountryCode'        "$p.Buyer_PostalAddress_CountryCode"
            WV $w 'Buyer_PostalAddress_CounryName'         "$p.Buyer_PostalAddress_CounryName"
            WV $w 'Buyer_PostalAddress_Region'             "$p.Buyer_PostalAddress_Region"
            WV $w 'Buyer_PostalAddress_City'               "$p.Buyer_PostalAddress_City"
            WV $w 'Buyer_PostalAddress_StreetHouse'        "$p.Buyer_PostalAddress_StreetHouse"
            WV $w 'Seler_Name'                             "$p.Seler_Name"
            WV $w 'Seler_PostalAddress_CountryCode'        "$p.Seler_PostalAddress_CountryCode"
            WV $w 'Seler_PostalAddress_CounryName'         "$p.Seler_PostalAddress_CounryName"
            WV $w 'Seler_PostalAddress_Region'             "$p.Seler_PostalAddress_Region"
            WV $w 'Seler_PostalAddress_City'               "$p.Seler_PostalAddress_City"
            WV $w 'Seler_PostalAddress_StreetHouse'        "$p.Seler_PostalAddress_StreetHouse"
            WV $w 'Consignor_OrganizationName'             "$p.Consignor_OrganizationName"
            WV $w 'Consignor_Address_CountryCode'          "$p.Consignor_Address_CountryCode"
            WV $w 'Consignor_Address_CounryName'           "$p.Consignor_Address_CounryName"
            WV $w 'Consignor_Address_Region'               "$p.Consignor_Address_Region"
            WV $w 'Consignor_Address_City'                 "$p.Consignor_Address_City"
            WV $w 'Consignor_Address_StreetHouse'          "$p.Consignor_Address_StreetHouse"
            WV $w 'Consignee_OrganizationName'             "$p.Consignee_OrganizationName"
            WV $w 'Consignee_OGRN'                         "$p.Consignee_OGRN"
            WV $w 'Consignee_INN'                          "$p.Consignee_INN"
            WV $w 'Consignee_KPP'                          "$p.Consignee_KPP"
            WV $w 'Consignee_Address_PostalCode'           "$p.Consignee_Address_PostalCode"
            WV $w 'Consignee_Address_CountryCode'          "$p.Consignee_Address_CountryCode"
            WV $w 'Consignee_Address_CounryName'           "$p.Consignee_Address_CounryName"
            WV $w 'Consignee_Address_Region'               "$p.Consignee_Address_Region"
            WV $w 'Consignee_Address_City'                 "$p.Consignee_Address_City"
            WV $w 'Consignee_Address_StreetHouse'          "$p.Consignee_Address_StreetHouse"

            # Array InvoiceGoods
            for ($i=1; $i -le 100; $i++) {
                $arrPref = "$p.InvoiceGoods_$i"
                if (-not (HasKey "$arrPref.GoodsCode")) { break }
                $w.WriteStartElement('InvoiceGoods')
                WV $w 'GoodsCode'                               "$arrPref.GoodsCode"
                WV $w 'GoodsDescription'                        "$arrPref.GoodsDescription"
                WV $w 'GoodsQuantity'                           "$arrPref.GoodsQuantity"
                WV $w 'MeasureUnitQualifierName'                "$arrPref.MeasureUnitQualifierName"
                WV $w 'GrossWeightQuantity'                     "$arrPref.GrossWeightQuantity"
                WV $w 'NetWeightQuantity'                       "$arrPref.NetWeightQuantity"
                WV $w 'Price'                                   "$arrPref.Price"
                WV $w 'TotalCost'                               "$arrPref.TotalCost"
                WV $w 'OriginCountryCode'                       "$arrPref.OriginCountryCode"
                WV $w 'AdditionalGoodsDescription_Manufacturer' "$arrPref.AdditionalGoodsDescription_Manufacturer"
                WV $w 'AdditionalGoodsDescription_TradeMark'    "$arrPref.AdditionalGoodsDescription_TradeMark"
                WV $w 'AdditionalGoodsDescription_GoodsMark'    "$arrPref.AdditionalGoodsDescription_GoodsMark"
                WV $w 'AdditionalGoodsDescription_GoodsModel'   "$arrPref.AdditionalGoodsDescription_GoodsModel"
                $w.WriteEndElement()
            }
        }

        "AltaE2PACK" {
            WV $w 'GrossWeightQuantity'                         "$p.GrossWeightQuantity"
            WV $w 'NetWeightQuantity'                           "$p.NetWeightQuantity"
            WV $w 'Consignor_OrganizationName'                  "$p.Consignor_OrganizationName"
            WV $w 'Consignor_ShortName'                         "$p.Consignor_ShortName"
            WV $w 'Consignor_Address_CountryCode'               "$p.Consignor_Address_CountryCode"
            WV $w 'Consignor_Address_CounryName'                "$p.Consignor_Address_CounryName"
            WV $w 'Consignor_Address_Region'                    "$p.Consignor_Address_Region"
            WV $w 'Consignor_Address_City'                      "$p.Consignor_Address_City"
            WV $w 'Consignor_Address_StreetHouse'               "$p.Consignor_Address_StreetHouse"
            WV $w 'Consignee_OrganizationName'                  "$p.Consignee_OrganizationName"
            WV $w 'Consignee_ShortName'                         "$p.Consignee_ShortName"
            WV $w 'Consignee_OGRN'                              "$p.Consignee_OGRN"
            WV $w 'Consignee_INN'                               "$p.Consignee_INN"
            WV $w 'Consignee_KPP'                               "$p.Consignee_KPP"
            WV $w 'Consignee_Address_PostalCode'                "$p.Consignee_Address_PostalCode"
            WV $w 'Consignee_Address_CountryCode'               "$p.Consignee_Address_CountryCode"
            WV $w 'Consignee_Address_CounryName'                "$p.Consignee_Address_CounryName"
            WV $w 'Consignee_Address_Region'                    "$p.Consignee_Address_Region"
            WV $w 'Consignee_Address_City'                      "$p.Consignee_Address_City"
            WV $w 'Consignee_Address_StreetHouse'               "$p.Consignee_Address_StreetHouse"
            WV $w 'DeliveryTerms_DeliveryPlace'                 "$p.DeliveryTerms_DeliveryPlace"
            WV $w 'DeliveryTerms_DeliveryTermsNumericCode'      "$p.DeliveryTerms_DeliveryTermsNumericCode"
            WV $w 'DeliveryTerms_DeliveryTermsStringCode'       "$p.DeliveryTerms_DeliveryTermsStringCode"
            WV $w 'DeliveryTerms_Contract_PrDocumentName'       "$p.DeliveryTerms_Contract_PrDocumentName"
            WV $w 'DeliveryTerms_Contract_PrDocumentNumber'     "$p.DeliveryTerms_Contract_PrDocumentNumber"
            WV $w 'DeliveryTerms_Contract_PrDocumentDate'       "$p.DeliveryTerms_Contract_PrDocumentDate"
            WV $w 'DeliveryTerms_Invoice_PrDocumentName'        "$p.DeliveryTerms_Invoice_PrDocumentName"
            WV $w 'DeliveryTerms_Invoice_PrDocumentNumber'      "$p.DeliveryTerms_Invoice_PrDocumentNumber"
            WV $w 'DeliveryTerms_Invoice_PrDocumentDate'        "$p.DeliveryTerms_Invoice_PrDocumentDate"
            WV $w 'DeliveryTerms_Registration_PrDocumentName'   "$p.DeliveryTerms_Registration_PrDocumentName"
            WV $w 'DeliveryTerms_Registration_PrDocumentNumber' "$p.DeliveryTerms_Registration_PrDocumentNumber"
            WV $w 'DeliveryTerms_Registration_PrDocumentDate'   "$p.DeliveryTerms_Registration_PrDocumentDate"

            # Array Goods
            for ($i=1; $i -le 100; $i++) {
                $arrPref = "$p.Goods_$i"
                if (-not (HasKey "$arrPref.GoodsDescription")) { break }
                $w.WriteStartElement('Goods')
                WV $w 'GoodsDescription'           "$arrPref.GoodsDescription"
                WV $w 'GoodsQuantity'              "$arrPref.GoodsQuantity"
                WV $w 'GrossWeightQuantity'        "$arrPref.GrossWeightQuantity"
                WV $w 'NetWeightQuantity'          "$arrPref.NetWeightQuantity"
                # Corrected: mapped to raw index 05 as PakingQuantity is flat under Goods_i in primary.md
                $w.WriteStartElement('PackingInfo')
                WV $w 'PakingQuantity'             "$arrPref.05"
                $w.WriteEndElement()
                $w.WriteEndElement()
            }

            # Array TransportMeans
            for ($i=1; $i -le 20; $i++) {
                $arrPref = "$p.TransportMeans_$i"
                if (-not (HasKey "$arrPref.Number")) { break }
                $w.WriteStartElement('TransportMeans')
                WV $w 'Number'          "$arrPref.Number"
                WV $w 'ModeCode'        "$arrPref.ModeCode"
                WV $w 'NationalityCode' "$arrPref.NationalityCode"
                WV $w 'MoverIndicator'  "$arrPref.MoverIndicator"
                $w.WriteEndElement()
            }
        }

        "AltaE3CMR" {
            WV $w 'LanguageCode'                              "$p.LanguageCode"
            WV $w 'CMR_Choice'                                "$p.CMR_Choice"
            WV $w 'RegistrationDocument_RegID'                "$p.RegistrationDocument_RegID"
            WV $w 'RegistrationDocument_DateInf'              "$p.RegistrationDocument_DateInf"
            WV $w 'RegistrationDocument_Place'                "$p.RegistrationDocument_Place"
            WV $w 'TrakingCargo_TakingCargoDate'              "$p.TrakingCargo_TakingCargoDate"
            WV $w 'TrakingCargo_TakingCargoPlace_CountryCode' "$p.TrakingCargo_TakingCargoPlace_CountryCode"
            WV $w 'TrakingCargo_TakingCargoPlace_CounryName'  "$p.TrakingCargo_TakingCargoPlace_CounryName"
            WV $w 'DeliveryPlace_CountryCode'                 "$p.DeliveryPlace_CountryCode"
            WV $w 'DeliveryPlace_CounryName'                  "$p.DeliveryPlace_CounryName"
            WV $w 'DeliveryTerms_DeliveryPlace'               "$p.DeliveryTerms_DeliveryPlace"
            WV $w 'DeliveryTerms_DeliveryTermsStringCode'     "$p.DeliveryTerms_DeliveryTermsStringCode"
            WV $w 'GoodsQuantity'                             "$p.GoodsQuantity"
            WV $w 'CMRGoodsWeight_GrossWeightQuantity'        "$p.CMRGoodsWeight_GrossWeightQuantity"
            WV $w 'CMRTransport_PrimeMoverStateSignID'        "$p.CMRTransport_PrimeMoverStateSignID"
            WV $w 'CMRTransport_TrailerStateSignID'           "$p.CMRTransport_TrailerStateSignID"
            WV $w 'Consignor_NameInf'                         "$p.Consignor_NameInf"
            WV $w 'Consignor_ShortName'                       "$p.Consignor_ShortName"
            WV $w 'Consignor_PostalAddress_CountryCode'       "$p.Consignor_PostalAddress_CountryCode"
            WV $w 'Consignor_Address_CounryName'              "$p.Consignor_Address_CounryName"
            WV $w 'Consignor_Address_Region'                  "$p.Consignor_Address_Region"
            WV $w 'Consignor_Address_City'                    "$p.Consignor_Address_City"
            WV $w 'Consignor_Address_StreetHouse'             "$p.Consignor_Address_StreetHouse"
            WV $w 'Consignor_Guarantee_OrganizationName'      "$p.Consignor_Guarantee_OrganizationName"
            WV $w 'Consignor_Guarantee_ShortName'             "$p.Consignor_Guarantee_ShortName"
            WV $w 'Consignor_Guarantee_Address_CountryCode'   "$p.Consignor_Guarantee_Address_CountryCode"
            WV $w 'Consignor_Guarantee_Address_CounryName'    "$p.Consignor_Guarantee_Address_CounryName"
            WV $w 'Consignor_Guarantee_Address_Region'        "$p.Consignor_Guarantee_Address_Region"
            WV $w 'Consignor_Guarantee_Address_City'          "$p.Consignor_Guarantee_Address_City"
            WV $w 'Consignor_Guarantee_Address_StreetHouse'   "$p.Consignor_Guarantee_Address_StreetHouse"
            WV $w 'Consignee_NameInf'                         "$p.Consignee_NameInf"
            WV $w 'Consignee_ShortName'                       "$p.Consignee_ShortName"
            WV $w 'Consignee_OGRNID'                          "$p.Consignee_OGRNID"
            WV $w 'Consignee_INNID'                           "$p.Consignee_INNID"
            WV $w 'Consignee_KPPCode'                         "$p.Consignee_KPPCode"
            WV $w 'Consignee_PostalAddress_PostalCode'        "$p.Consignee_PostalAddress_PostalCode"
            WV $w 'Consignee_PostalAddress_CountryCode'       "$p.Consignee_PostalAddress_CountryCode"
            WV $w 'Consignee_Address_CounryName'              "$p.Consignee_Address_CounryName"
            WV $w 'Consignee_Address_Region'                  "$p.Consignee_Address_Region"
            WV $w 'Consignee_Address_City'                    "$p.Consignee_Address_City"
            WV $w 'Consignee_Address_StreetHouse'             "$p.Consignee_Address_StreetHouse"

            # Corrected: CMRGoods has exactly 3 fields (GoodsNumeric=01, GoodsDescription=02, PakingQuantity=03) in primary_schema.md
            for ($i=1; $i -le 100; $i++) {
                $arrPref = "$p.CMRGoods_$i"
                if (-not (HasKey "$arrPref.02")) { break }
                $w.WriteStartElement('CMRGoods')
                WV $w 'GoodsDescription'        "$arrPref.02"
                WV $w 'GoodsNumeric'            "$arrPref.01"
                $w.WriteStartElement('GoodsPackingInfo')
                WV $w 'PakingQuantity'          "$arrPref.03"
                $w.WriteEndElement()
                $w.WriteEndElement()
            }
        }

        "AltaPaymentOrder" {
            WriteEl $w 'DocumentCode' '04023'
            WV $w 'PaymentModeCode'                    "$p.PaymentModeCode"
            WV $w 'PaymentAmount'                      "$p.PaymentAmount"
            WV $w 'TransactionKind'                    "$p.TransactionKind"
            WV $w 'Priority'                           "$p.Priority"
            WV $w 'Purpose'                            "$p.Purpose"
            WV $w 'ValueSpelledOut'                    "$p.ValueSpelledOut"
            WV $w 'DocumentReference_PrDocumentNumber' "$p.DocumentReference_PrDocumentNumber"
            WV $w 'DocumentReference_PrDocumentDate'   "$p.DocumentReference_PrDocumentDate"
            WV $w 'Payer_OrganizationName'             "$p.Payer_OrganizationName"
            WV $w 'Payer_INN'                          "$p.Payer_INN"
            WV $w 'Payer_KPP'                          "$p.Payer_KPP"
            WV $w 'Payer_Bank_BankName'                "$p.Payer_Bank_BankName"
            WV $w 'Payee_OrganizationName'             "$p.Payee_OrganizationName"
            WV $w 'Payee_Bank_BankName'                "$p.Payee_Bank_BankName"
            
            # Corrected: PersonSurname (16) and PersonName (17) are flat under payment_order_n in primary.md
            $w.WriteStartElement('PayerSign')
            WV $w 'PersonSurname' "$p.16"
            WV $w 'PersonName'    "$p.17"
            $w.WriteEndElement()
        }

        "AltaServiceInvoice" {
            # Corrected: using exact 1-based index numbers to completely bypass field collisions under Service Invoice
            WV $w 'DocumentSign'                                                   "$p.01"
            WV $w 'TotalServiceCost'                                               "$p.02"
            WV $w 'Currency'                                                       "$p.03"
            WV $w 'ServiceProvider_Name'                                           "$p.04"
            
            $w.WriteStartElement('ServiceProvider_PaymentRequisitions')
            WV $w 'BankName' "$p.05"
            $w.WriteEndElement()
            
            WV $w 'ContractDetails_PrDocumentNumber'                               "$p.06"
            WV $w 'ContractDetails_PrDocumentDate'                                 "$p.07"

            $w.WriteStartElement('PaymentDocument')
            WV $w 'PrDocumentNumber' "$p.08"
            WV $w 'PrDocumentDate'   "$p.09"
            $w.WriteEndElement()
            
            WV $w 'Registration_PrDocumentName'                                    "$p.10"
            WV $w 'Registration_PrDocumentNumber'                                  "$p.11"
            WV $w 'Registration_PrDocumentDate'                                    "$p.12"

            $w.WriteStartElement('Consignor_SubjectAddressDetails')
            WV $w 'PostalCode'  "$p.14"
            WV $w 'CountryCode' "$p.15"
            WV $w 'CounryName'  "$p.16"
            WV $w 'Region'      "$p.17"
            WV $w 'Town'        "$p.18"
            WV $w 'StreetHouse' "$p.19"
            $w.WriteEndElement()

            $w.WriteStartElement('Consignee_SubjectAddressDetails')
            WV $w 'PostalCode'  "$p.24"
            WV $w 'CountryCode' "$p.25"
            WV $w 'CounryName'  "$p.26"
            WV $w 'Region'      "$p.27"
            WV $w 'Town'        "$p.28"
            WV $w 'StreetHouse' "$p.29"
            WV $w 'House'       "$p.30"
            WV $w 'Room'        "$p.31"
            $w.WriteEndElement()

            WV $w 'Signature_Choice'                                               "$p.32"
            WV $w 'SignatureDirectorChiefAccountant_Director_PersonSurname'        "$p.36"
            WV $w 'SignatureDirectorChiefAccountant_Director_PersonName'           "$p.37"
            WV $w 'SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname' "$p.39"
            WV $w 'SignatureDirectorChiefAccountant_ChiefAccountant_PersonName'    "$p.40"

            # Array ServiceDescription
            for ($i=1; $i -le 50; $i++) {
                $arrPref = "$p.ServiceDescription_$i"
                if (-not (HasKey "$arrPref.GoodsDescription")) { break }
                $w.WriteStartElement('ServiceDescription')
                WV $w 'GoodsDescription'     "$arrPref.GoodsDescription"
                WV $w 'CurrencyCode'         "$arrPref.CurrencyCode"
                WV $w 'ServiceName'          "$arrPref.ServiceName"
                WV $w 'TaxRate'              "$arrPref.TaxRate"
                WV $w 'TaxSum'               "$arrPref.TaxSum"
                WV $w 'ServiceCost_Amount'   "$arrPref.ServiceCost_Amount"
                WV $w 'ServiceCost_Currency' "$arrPref.ServiceCost_Currency"
                $w.WriteEndElement()
            }
        }

        "AltaFreeDoc" {
            $code = $doc.Code
            WriteEl $w 'DocumentCode' $code
            WV $w 'DocumentHead_DocumentName'   "$p.DocumentHead_DocumentName"
            WV $w 'DocumentHead_DocumentDate'   "$p.DocumentHead_DocumentDate"
            WV $w 'DocumentHead_DocumentNumber' "$p.DocumentHead_DocumentNumber"

            # Write the nested TextPara inside the DocumentBody_TextSection grouping tag
            $w.WriteStartElement('DocumentBody_TextSection')
            WV $w 'TextPara' "$p.TextPara"
            $w.WriteEndElement()
        }
    }

    $w.WriteEndElement() # Close Root
    $w.WriteEndDocument()
    $w.Close()

    # Scan the generated file for unresolved links (using correct encoding and regex for spaces)
    $xmlLines = [System.IO.File]::ReadAllLines($outPath, $enc)
    $hasLinks = $false
    for ($lineIdx = 0; $lineIdx -lt $xmlLines.Count; $lineIdx++) {
        $currentLine = $xmlLines[$lineIdx]
        # Match everything after 'link:' up to the closing comment characters ' -->'
        if ($currentLine -match 'RESOLVE_LINK:\s*(link:.*?)\s*-->') {
            $linkPath = $Matches[1].Trim()
            $xmlLineNum = $lineIdx + 1
            Write-Host "Generated: $fileName | Line $xmlLineNum | $linkPath" -ForegroundColor Yellow
            $hasLinks = $true
        }
    }

    if (-not $hasLinks) {
        Write-Host "Generated: $fileName"
    }
}

# --------------------------------------------------------------
# FINAL VERIFICATION
# --------------------------------------------------------------
if ($Errors.Count -gt 0) {
    Write-Host ""
    Write-Host "Generation completed with WARNINGS/ERRORS:" -ForegroundColor Yellow
    $Errors | ForEach-Object { Write-Host $_ }
    Write-Host "Total warnings: $($Errors.Count)"
    exit 1
}

Write-Host "SUCCESS: All XML files generated without blocking errors." -ForegroundColor Green
exit 0
