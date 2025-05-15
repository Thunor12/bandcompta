
export function DropDown({ name, isRequired, choices }) {
    return (
        <select name={name} required={isRequired === true}>
            {choices.map((opt) => <option value={opt.value}>{opt.label}</option>)}
        </select>
    );
}
