import { start } from "yew-material-scripts";

start(async () => {
    return [await import("../pkg"), await import("../config-features")];
});

