interface FileDetail {
    path: string;
    name: string;
    size: number;
}

interface ObjectDetail {
    last_modified: string;
    etag: string;
    size: number;
    key: string;
}

interface ChannelBytes {
    value: number;
}