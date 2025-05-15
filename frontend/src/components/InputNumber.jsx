// TODO look at eslint

import { useState } from "react";

export function InputNumber({ name, placeHolder, isRequired, defaultValue = 0 }) {

    return (
        <input
            name={name}
            placeholder={placeHolder}
            type="number"
            defaultValue={defaultValue}
            required={isRequired === true}
        />
    );
}
