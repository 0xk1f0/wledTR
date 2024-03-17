import { Store } from '@tauri-apps/plugin-store';
import type { Device, StoreData } from '$lib/types/store';

class StorageHandler {
    #path: string;

    constructor(path: string) {
        this.#path = path;
    }

    private async load(store: Store): Promise<StoreData> {
        let query = await store.get<StoreData>('config');
        if (query === null) {
            query = { devices: [] };
        }
        return query;
    }

    private add(store: Store) {
        const single = async (store: Store, device: Device) => {
            let data = await store.get<StoreData>('config') || { devices: [] };
            data.devices.push(device)
            await store.set('config', data);
            return {
                save: () => {
                    return store.save();
                }
            };
        }

        const multiple = async (store: Store, devices: Device[]) => {
            let data = await store.get<StoreData>('config') || { devices: [] };
            for (let device of devices) {
                data.devices.push(device);
            }
            await store.set('config', data);
            return {
                save: () => {
                    return store.save();
                }
            };
        }

        return {
            single: (device: Device) => { return single(store, device) },
            multiple: (devices: Device[]) => { return multiple(store, devices) }
        };
    }

    private remove(store: Store) {
        const single = async (store: Store, device: Device) => {
            let data = await store.get<StoreData>('config') || { devices: [] };
            data.devices.splice(data.devices.indexOf(device), 1);
            await store.set('config', data);
            return {
                save: () => {
                    return store.save();
                }
            };
        }

        const multiple = async (store: Store, devices: Device[]) => {
            let data = await store.get<StoreData>('config') || { devices: [] };
            for (let device of devices) {
                data.devices.splice(data?.devices.indexOf(device), 1);
            }
            await store.set('config', data);
            return {
                save: () => {
                    return store.save();
                }
            };
        }

        return {
            single: (device: Device) => { return single(store, device) },
            multiple: (devices: Device[]) => { return multiple(store, devices) }
        };
    }

    public open() {
        let store = new Store(this.#path);
        return {
            load: () => { return this.load(store) },
            add: () => { return this.add(store) },
            remove: () => { return this.remove(store) },
            clear: () => { return store.clear() },
            save: () => { return store.save() }
        };
    }
}

export default StorageHandler;