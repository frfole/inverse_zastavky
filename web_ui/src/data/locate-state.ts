import {BaseStation, ChainStation} from "../model/model.ts";

export interface LocateState {
    offset: number,
    limit: number,
    selectedIdx: number,
    chainStations: ChainStation[],
    baseStations: BaseStation[],
}
