import {Mode} from "../../model/model.ts";
import "./ModeSwitcher.css"
import {useContext} from "react";
import {AppContext, AppDispatchContext} from "../../data/app-context.ts";
import {ActionType} from "../../data/app-reducer.ts";

function ModeSwitcher() {
    const appState = useContext(AppContext)
    const appDispatch = useContext(AppDispatchContext)

    return (
        <div className="wrapper">
            {Object.entries(Mode).map((mode, index) =>
                <button className={appState.mode === mode[1] ? "active-mode" : "inactive-mode"}
                        key={index}
                        onClick={() => {
                            appDispatch({
                                type: ActionType.ModeChange,
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