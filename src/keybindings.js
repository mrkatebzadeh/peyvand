window.appState = {
  mode: "Normal",
  commandBuffer: "",
  keyBuffer: [],
  keyTimeout: null,
};

const sendCommand = (cmd) => window.ipc.postMessage(cmd);

const resetKeyBuffer = () => {
  window.appState.keyBuffer = [];
  if (window.appState.keyTimeout) {
    clearTimeout(window.appState.keyTimeout);
    window.appState.keyTimeout = null;
  }
};

function handleNormalModeKey(e) {
  let key = e.key;
  if (e.ctrlKey) key = "C-" + key;

  window.appState.keyBuffer.push(key);
  if (window.appState.keyTimeout) clearTimeout(window.appState.keyTimeout);
  window.appState.keyTimeout = setTimeout(resetKeyBuffer, 500);

  const seq = window.appState.keyBuffer.join("");

  switch (seq) {
    case "gg":
      sendCommand("scroll-top");
      resetKeyBuffer();
      e.preventDefault();
      return;
    case "gG":
      sendCommand("scroll-bottom");
      resetKeyBuffer();
      e.preventDefault();
      return;
    case "C-d":
      sendCommand("scroll-half-down");
      resetKeyBuffer();
      e.preventDefault();
      return;
    case "C-u":
      sendCommand("scroll-half-up");
      resetKeyBuffer();
      e.preventDefault();
      return;
  }

  switch (e.key) {
    case "h":
      sendCommand("go-back");
      e.preventDefault();
      break;
    case "l":
      sendCommand("go-forward");
      e.preventDefault();
      break;
    case "j":
      sendCommand("scroll-down");
      e.preventDefault();
      break;
    case "k":
      sendCommand("scroll-up");
      e.preventDefault();
      break;
    case "i":
      window.appState.mode = "Insert";
      sendCommand("mode-insert");
      e.preventDefault();
      break;
    case ":":
      window.appState.mode = "Command";
      window.appState.commandBuffer = "";
      sendCommand("mode-command");
      e.preventDefault();
      break;
  }
}

function handleInsertModeKey(e) {
  if (e.key === "Escape") {
    window.appState.mode = "Normal";
    sendCommand("mode-normal");
    resetKeyBuffer();
    e.preventDefault();
  }
}

function handleCommandModeKey(e) {
  if (e.key === "Enter") {
    sendCommand("command:" + window.appState.commandBuffer);
    window.appState.commandBuffer = "";
    window.appState.mode = "Normal";

    sendCommand("mode-normal");
    resetKeyBuffer();
    e.preventDefault();
  } else if (e.key.length === 1 && !e.ctrlKey && !e.metaKey) {
    window.appState.commandBuffer += e.key;
    e.preventDefault();
  }
}

document.addEventListener("keydown", (e) => {
  e.stopPropagation();

  if (e.key === "Escape" && window.appState.mode !== "Normal") {
    window.appState.mode = "Normal";
    sendCommand("mode-normal");
    window.appState.commandBuffer = "";
    resetKeyBuffer();
    e.preventDefault();
    return;
  }

  switch (window.appState.mode) {
    case "Normal":
      handleNormalModeKey(e);
      break;
    case "Insert":
      handleInsertModeKey(e);
      break;
    case "Command":
      handleCommandModeKey(e);
      break;
  }
});
