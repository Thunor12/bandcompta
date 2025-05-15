// TODO look at eslint

import { useState } from "react";

export function InputFile({ name, placeHolder, isRequired }) {

    return (
        <input
            name={name}
            placeholder={placeHolder}
            type="file"
            required={isRequired === true}
        />
    );
}
