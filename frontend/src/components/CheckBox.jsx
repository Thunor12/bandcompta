
export function CheckBox({ name, isRequired, label, defaultValue = false }) {
    return (
        <label>
            <input type="checkbox" name={name} defaultValue={defaultValue} required={isRequired === true} />
            {label}
        </label>
    );
}
