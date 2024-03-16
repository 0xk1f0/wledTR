export type Device = {
    host: string;
    mdns: boolean;
};

export type StoreData = {
    devices: Device[];
};
