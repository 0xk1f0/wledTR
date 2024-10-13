export type Color = [number, number, number];

export type WifiData = {
    bssid: string;
    rssi: number;
    signal: number;
    channel: number;
};

export type Segments = {
    start: number;
    stop: number;
    length: number;
    col: Color[];
};

export type StateResponse = {
    on: boolean;
    bri: number;
    ps: number;
    seg: Segments[];
};

export type InfoResponse = {
    ver: string;
    name: string;
    arch: string;
    core: string;
    freeheap: number;
    uptime: number;
    opt: number;
    brand: string;
    product: string;
    mac: string;
    ip: string;
    wifi: WifiData;
};

export type NormalResponse = {
    success: boolean;
};

export type PowerResponse = {
    on: boolean;
};
