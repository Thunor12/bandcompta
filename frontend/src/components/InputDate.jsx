import { useState } from "react";

export function InputDate({ name, placeHolder, isRequired }) {

    return (
        <input
            name={name}
            placeholder={placeHolder}
            type="date"
            required={isRequired === true}
        />
    );
}
