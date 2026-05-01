const { invoke } = window.__TAURI__.core;

const ZOOM_STEP = 0.1;
const MAX_ZOOM = 3.0;
const MIN_ZOOM = 0.5;
const DEFAULT_ZOOM = 1.0;

let currentZoom = DEFAULT_ZOOM;

// Apply and save zoom
const applyAndSaveZoom = async (newZoom) => {
	// Clamp the zoom level
	currentZoom = Math.max(MIN_ZOOM, Math.min(newZoom, MAX_ZOOM));

	// Apply the zoom
	try {
		await invoke("set_zoom", { zoom: currentZoom });
		console.log(`Applied zoom level: ${(currentZoom * 100).toFixed(0)}%`);
	} catch (err) {
		console.error("Failed to set zoom:", err);
	}

	// Save the zoom level
	try {
		await invoke("save_zoom", { zoom: currentZoom });
		console.log(`Saved zoom level: ${(currentZoom * 100).toFixed(0)}%`);
	} catch (err) {
		console.error("Failed to save zoom:", err);
	}
};

// Listen for zoom changes via keyboard shortcuts
window.addEventListener("keydown", (event) => {
	if (event.ctrlKey) {
		let newZoom = currentZoom;
		if (event.key === "+" || event.key === "=") {
			newZoom += ZOOM_STEP;
			event.preventDefault();
		} else if (event.key === "-") {
			newZoom -= ZOOM_STEP;
			event.preventDefault();
		} else if (event.key === "0") {
			newZoom = DEFAULT_ZOOM;
			event.preventDefault();
		}

		if (newZoom !== currentZoom) {
			applyAndSaveZoom(newZoom);
		}
	}
});

// Listen for zoom changes via mouse wheel
window.addEventListener(
	"wheel",
	(event) => {
		if (event.ctrlKey) {
			event.preventDefault();
			let newZoom = currentZoom;
			if (event.deltaY < 0) {
				newZoom += ZOOM_STEP;
			} else {
				newZoom -= ZOOM_STEP;
			}

			applyAndSaveZoom(newZoom);
		}
	},
	{ passive: false },
);

// Load initial zoom level from store on startup
window.addEventListener("DOMContentLoaded", async () => {
	try {
		const savedZoom = await invoke("load_zoom");
		if (savedZoom) {
			currentZoom = savedZoom;
			console.log(`Restored zoom level: ${(currentZoom * 100).toFixed(0)}%`);
			// Apply the zoom immediately
			await invoke("set_zoom", { zoom: currentZoom });
		} else {
			console.log("No saved zoom level found, using default.");
		}
	} catch (err) {
		console.error("Failed to load zoom on startup:", err);
	}
});
