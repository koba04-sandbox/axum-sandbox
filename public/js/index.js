const btn = document.getElementById("increment");
const text = document.getElementById("count");
document.getElementById("increment").addEventListener("click", () => {
    fetch("/api", { method: "POST" }).then(res => res.json()).then(res => {
        text.textContent = res.count;
    })
});