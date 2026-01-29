/**
 * popup.js — Переключатель состояния Хобота
 *
 * ЗАМЫСЕЛ:
 * Popup открывается по клику на иконку. Показывает ОДНУ кнопку:
 * ⏸️ ПАУЗА (красная) = Хобот НА ПАУЗЕ.
 * ▶️ РАБОТА (зеленая) = Хобот АКТИВЕН.
 *
 * ЛОГИКА:
 * 1. При открытии — читает состояние из chrome.storage.local.
 * 2. Клик → меняет состояние → пишет в chrome.storage.local.
 * 3. Content.js сам реагирует через onChanged:
 *    - в ПАУЗЕ: выключает MutationObserver (ничего не читаем со страницы),
 *    - в РАБОТЕ: включает наблюдение и поиск директив в code-block (<pre>).
 */

const btn = document.getElementById('toggle-btn');
let isPaused = true; // Дефолтное значение

// --- 1. Инициализация при открытии ---
// Читаем реальное состояние из хранилища.
chrome.storage.local.get(['hobotPaused'], (result) => {
    // Если ключа нет (первый запуск) — считаем true (пауза).
    // Если ключ есть — берем его.
    isPaused = result.hobotPaused !== undefined ? result.hobotPaused : true;
    updateButtonView();
});

// --- 2. Обработка клика ---
btn.onclick = () => {
    isPaused = !isPaused;

    // Прямая запись в хранилище (разрешение "storage" в manifest.json)
    chrome.storage.local.set({ hobotPaused: isPaused });

    updateButtonView();
};

// --- 3. Отрисовка кнопки ---
function updateButtonView() {
    if (isPaused) {
        btn.textContent = '⏸️ ПАУЗА';
        btn.className = 'paused';   // CSS: красный
    } else {
        btn.textContent = '▶️ РАБОТА';
        btn.className = 'working';  // CSS: зеленый
    }
}
