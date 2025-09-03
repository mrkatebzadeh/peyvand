window.appState = {
    mode: 'Normal',
    commandBuffer: ''
};
document.addEventListener('keydown', (e) => {
    e.stopPropagation();

    const mode = window.appState.mode;
    if (e.key === 'Escape') {
        if (mode !== 'Normal') {
            window.appState.mode = 'Normal';
            window.ipc.postMessage('mode-normal');
            window.appState.commandBuffer = '';
            e.preventDefault();
        }
        return;
    }
    if (mode === 'Normal') {
        switch (e.key) {
            case 'h':
                window.ipc.postMessage('go-back');
                e.preventDefault();
                break;
            case 'l':
                window.ipc.postMessage('go-forward');
                e.preventDefault();
                break;
            case 'j':
                window.ipc.postMessage('scroll-down');
                e.preventDefault();
                break;
            case 'k':
                window.ipc.postMessage('scroll-up');
                e.preventDefault();
                break;
            case 'i':
                window.appState.mode = 'Insert';
                window.ipc.postMessage('mode-insert');
                e.preventDefault();
                break;
            case ':':
                window.appState.mode = 'Command';
                window.appState.commandBuffer = '';
                window.ipc.postMessage('mode-command');
                e.preventDefault();
                break;
        }
    } else if (mode === 'Insert') {

    if (e.key === 'Escape') {

            window.ipc.postMessage('mode-normal');
    }
    } else if (mode === 'Command') {
        if (e.key === 'Enter') {
        window.ipc.postMessage('command:' + window.appState.commandBuffer);
        window.appState.commandBuffer = '';
        e.preventDefault();
    } else if (e.key.length === 1 && !e.ctrlKey && !e.metaKey) {
                        if (!window.appState.commandBuffer) window.appState.commandBuffer = '';

        window.appState.commandBuffer += e.key;
        e.preventDefault();
    }
    }
});
