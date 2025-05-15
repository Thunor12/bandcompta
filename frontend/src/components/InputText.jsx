// TODO look at eslint

import { useState } from "react";

export function InputText({ name, placeHolder, isRequired }) {

    return (
        <input
            name={name}
            placeholder={placeHolder}
            type="text"
            required={isRequired === true}
        />
    );
}
