import { dia, terminalIo } from "dia";
import { $start } from "./script/start.ts";

const app = dia({ io: terminalIo });

app.run($start);
