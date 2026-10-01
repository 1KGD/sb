export default function initializer() {
    return {
        onProgress: ({ current, total }) => {
            document.getElementById("loading-bytes").innerText = `Loaded ${Math.floor(current / 1000000 * 100) / 100}mb`;
            if (total) {
                document.getElementById("loading-progress").value = current / total;
            }
        },
        onComplete: () => {
            document.getElementById("loading-panel").style.display = "none";
        }
    }
}