export const formatBytes = (bytes: number | undefined) => {
    if (bytes === undefined) {
        return '0 B';
    }
    const units = ['B', 'KB', 'MB', 'GB'];
    let index = 0;

    while (bytes >= 1024 && index < units.length - 1) {
        bytes /= 1024;
        index++;
    }

    return `${bytes.toFixed(2)} ${units[index]}`;
}

export const formatTimeAgo = (lastTime: string) => {
    if (!lastTime) return '';
    const now = new Date();
    const date = new Date(lastTime);
    const seconds = Math.floor((now - date) / 1000);
    let interval = Math.floor(seconds / 31536000);
    if (interval > 1) return `${interval} years ago`;
    interval = Math.floor(seconds / 2592000);
    if (interval > 1) return `${interval} months ago`;
    interval = Math.floor(seconds / 86400);
    if (interval > 1) return `${interval} days ago`;
    interval = Math.floor(seconds / 3600);
    if (interval > 1) return `${interval} hours ago`;
    interval = Math.floor(seconds / 60);
    if (interval > 1) return `${interval} minutes ago`;
    return `${seconds} seconds ago`;
};

export const pathConcat = (parentPath: string, file: string) => {
    if (parentPath.includes('/')) {
        return `${parentPath}/${file}`;
    } else {
        return `${parentPath}\\${file}`;
    }
}

export const isText = (name: string) => name.startsWith("remote-send") && name.endsWith(".txt");

export const isZipped = (name: string) => name.startsWith("remote-send") && name.endsWith(".zip");
