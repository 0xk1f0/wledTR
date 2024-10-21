import { Store } from '@tauri-apps/plugin-store';
import type { Device, StoreData } from '$lib/types/store';

class StorageHandler {
    #path: string;

    constructor(path: string) {
        this.#path = path;
    }

    private async load(store: Promise<Store>): Promise<StoreData> {
        let query = await (await store).get<StoreData>('config');
        if (!query) {
            query = { devices: [] };
        }
        return query;
    }

    private add(store: Promise<Store>) {
        const single = async (store: Promise<Store>, device: Device) => {
            let data = (await (await store).get<StoreData>('config')) || { devices: [] };
            data.devices.push(device);
            await (await store).set('config', data);
            return {
                save: async () => {
                    return await (await store).save();
                }
            };
        };

        const multiple = async (store: Promise<Store>, devices: Device[]) => {
            let data = (await (await store).get<StoreData>('config')) || { devices: [] };
            for (let device of devices) {
                data.devices.push(device);
            }
            await (await store).set('config', data);
            return {
                save: async () => {
                    return await (await store).save();
                }
            };
        };

        return {
            single: async (device: Device) => {
                return await single(store, device);
            },
            multiple: async (devices: Device[]) => {
                return await multiple(store, devices);
            }
        };
    }

    private remove(store: Promise<Store>) {
        const single = async (store: Promise<Store>, device: Device) => {
            let data = (await (await store).get<StoreData>('config')) || { devices: [] };
            data.devices.splice(data.devices.indexOf(device), 1);
            await (await store).set('config', data);
            return {
                save: async () => {
                    return (await store).save();
                }
            };
        };

        const multiple = async (store: Promise<Store>, devices: Device[]) => {
            let data = (await (await store).get<StoreData>('config')) || { devices: [] };
            for (let device of devices) {
                data.devices.splice(data?.devices.indexOf(device), 1);
            }
            await (await store).set('config', data);
            return {
                save: async () => {
                    return (await store).save();
                }
            };
        };

        return {
            single: (device: Device) => {
                return single(store, device);
            },
            multiple: (devices: Device[]) => {
                return multiple(store, devices);
            }
        };
    }

    public open() {
        let store = Store.load(this.#path);
        return {
            load: () => {
                return this.load(store);
            },
            add: () => {
                return this.add(store);
            },
            remove: () => {
                return this.remove(store);
            },
            clear: async () => {
                return (await store).clear();
            },
            save: async () => {
                return (await store).save();
            }
        };
    }
}

export default StorageHandler;
