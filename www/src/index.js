import * as wasm from "web-rscript";
import Behave from "behave-js";

import "./style.css";

// const FETCH_OPTIONS = {
//   mode: ""
// };

const DEFAULT_OPTIONS = {
  mode: "text",
};

/**
 *
 * @param url The URL to read from
 * @param options Options for reading the file
 */
const readFile = async (url, options = DEFAULT_OPTIONS) => {
  const res = await fetch(url);
  switch (options.mode) {
    case "text":
      const text = await res.text();
      return text;
    case "json":
      const json = await res.json();
      return json;
    default:
      throw Error(`${options.mode} is not a valid mode`);
  }
};

const loadLibrary = async (reload) => {
  if (reload) {
    const library = await getGithubRepo(
      window.log,
      "bwhetherington",
      "rscript",
      "std",
      "std"
    );
    localStorage.cachedLibrary = JSON.stringify(library);
    return library;
  } else {
    const { cachedLibrary } = localStorage;
    if (cachedLibrary) {
      const library = JSON.parse(cachedLibrary);
      return library;
    } else {
      return loadLibrary(true);
    }
  }
};

const getGithubRepo = async (
  logger = () => {},
  user = "bwhetherington",
  repo = "rscript",
  name = "std",
  path = "std"
) => {
  const url = `https://api.github.com/repos/${user}/${repo}/contents/${path}`;

  const res = await fetch(url);
  const data = await res.json();

  const mod = {
    name,
    body: "",
    children: [],
  };

  for (const child of data) {
    if (child.type === "dir") {
      const newPath = `${path}/${child.name}`;
      const childMod = await getGithubRepo(
        logger,
        user,
        repo,
        child.name,
        newPath
      );
      mod.children.push(childMod);
    } else if (child.type === "file") {
      const modName = child.name.split(".")[0];
      const fileData = await readFile(child.url, { mode: "json" });
      const fileText = await readFile(fileData.download_url);
      logger("Read: " + fileData.download_url);
      const childMod = {
        name: modName,
        body: fileText,
        children: [],
      };
      mod.children.push(childMod);
    }
  }

  return mod;
};

const output = document.getElementById("output");

const LOG_REGEX = /(\[([a-zA-Z-_]+)\]:)?(.*)/;

window.log2 = () => {
  // // Check regex
  // const match = text.match(LOG_REGEX);

  // const messageType = match[2] || "info";
  // const messageContent = match[3];

  // const message = document.createElement("div");
  // message.className = "message " + messageType;
  // message.innerText = messageContent;
  // output.appendChild(message);
  return Date.now();
};

window.log = (text) => {
  // Check regex
  const match = text.match(LOG_REGEX);

  const messageType = match[2] || "info";
  const messageContent = match[3];

  const message = document.createElement("div");
  message.className = "message " + messageType;
  message.innerText = messageContent;
  output.appendChild(message);
  message.scrollIntoView();
};

const removeChildren = (node) => {
  while (node.firstChild) {
    node.removeChild(node.lastChild);
  }
};

const convertOutput = (output) => {
  // Check regex
  const match = output.match(LOG_REGEX);
  const messageType = match[2] || "info";
  const messageContent = match[3];
  const message = document.createElement("div");
  message.className = "message " + messageType;
  message.innerText = messageContent;
  return message;
};

const main = async () => {
  const textarea = document.getElementById("text-input");

  const existing = localStorage.source;
  if (existing) {
    textarea.value = existing;
  }

  // Initialize editor
  const editor = new Behave({
    textarea,
    tabSize: 2,
  });

  const data = await loadLibrary(false);
  const text = JSON.stringify(data);

  wasm.init(text);
  window.log("Complete!");

  const input = document.getElementById("text-input");
  const run = document.getElementById("run-input");

  run.disabled = false;

  run.onclick = () => {
    removeChildren(output);
    // Save source
    localStorage.source = input.value;

    const start = Date.now();
    const res = wasm.run("{\nimport std::prelude::_;\n" + input.value + "\n}");
    const stop = Date.now();
    const dt = stop - start;
    if (res != "[ok]:Result: None") {
      log(res);
    }
    log(`[end-line]:Script completed in ${dt}ms.`);
  };

  const clear = document.getElementById("clear-output");
  clear.onclick = () => {
    removeChildren(output);
  };
};

main().catch((err) => console.error(err));
