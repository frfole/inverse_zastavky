import {LatLngBounds} from "leaflet";

export enum Mode {
    Browse = 'browse',
    Editor = 'editor',
    Test = 'test'
}

export class BBox {
    lat_from: number;
    lat_to: number;
    lon_from: number;
    lon_to: number;

    constructor(lat_1: number, lat_2: number, lon_1: number, lon_2: number) {
        this.lat_from = Math.min(lat_1, lat_2)
        this.lat_to = Math.max(lat_1, lat_2)
        this.lon_from = Math.min(lon_1, lon_2)
        this.lon_to = Math.max(lon_1, lon_2)
    }

    static fromBounds(bounds: LatLngBounds): BBox {
        return new BBox(
            bounds.getNorthEast().lat,
            bounds.getSouthWest().lat,
            bounds.getNorthEast().lng,
            bounds.getSouthWest().lng,
        )
    }
}

export interface Station {
    stop_id: number,
    lat: number,
    lon: number,
    names: string[]
}
