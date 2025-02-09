import "./ModeSwitcher.css"
import {Mode} from "../../model/model.ts";
import {useContext} from "react";
import {AppContext, AppDispatchContext} from "../../data/app-context.ts";
import {ActionType} from "../../data/app-reducer.ts";

function ModeSwitcher() {
    const appState = useContext(AppContext)
    const appDispatch = useContext(AppDispatchContext)

    return (
        <div className="ModeSwitcher__wrapper">
            {Object.entries(Mode).map((mode, index) =>
                <button className={appState.mode === mode[1] ? "ModeSwitcher__active-mode" : "ModeSwitcher__inactive-mode"}
                        key={index}
                        onClick={() => {
                            appDispatch({
                                type: ActionType.ChangeMode,
                                mode: mode[1]
                            })
                        }}>
                    {mode[0]}
                </button>
            )}
        </div>
    )
}

export default ModeSwitcher