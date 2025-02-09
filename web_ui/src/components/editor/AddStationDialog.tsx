import "./AddStationDialog.css"
import ReactModal from "react-modal";

interface AddStationDialogProps {
    open: boolean
    onClose: () => void
    onAdd: (name: string) => void
}

export function AddStationDialog(props: AddStationDialogProps) {
    function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault()
        const formData = new FormData(event.currentTarget);
        const formJson = Object.fromEntries(formData.entries());
        const name = formJson.name as string;
        props.onAdd(name)
        props.onClose()
    }

    return (
        <ReactModal isOpen={props.open} onRequestClose={props.onClose} style={{
            content: {
                width: "fit-content",
                height: "fit-content"
            }
        }}>
            <form onSubmit={handleSubmit}>
                <input className="input-name" type="text" name="name" placeholder="Name" autoFocus required/>
                <input className="button-add" type="submit" value="Add"/>
            </form>
        </ReactModal>
    )
}
