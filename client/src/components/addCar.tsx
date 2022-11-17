import React from "react";
import { useState } from "react";
import { Box, TextField } from "@mui/material";
import { LoadingButton } from "@mui/lab";
import { createCar } from "../api/api";

const newOnChange = (
  regex: RegExp,
  setValue: React.Dispatch<React.SetStateAction<string>>,
  setValid: React.Dispatch<React.SetStateAction<boolean>>
) => {
  return (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    let value = e.target.value;
    setValid(regex.test(value));
    setValue(value);
  };
};

const AddCar = () => {
  const [vin, setVin] = useState("");
  const [isVinValid, setIsVinValid] = useState(true);
  const [make, setMake] = useState("");
  const [isMakeValid, setIsMakeValid] = useState(true);
  const [model, setModel] = useState("");
  const [isModelValid, setIsModelValid] = useState(true);
  const [year, setYear] = useState("");
  const [isYearValid, setIsYearValid] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [isDisabled, setIsDisabled] = useState(false);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsLoading(true);
    createCar({
      vin: vin,
      make: make,
      model: model,
      year: +year,
    }).then((res) => {
      const msg = res.ok
        ? "Created successfully"
        : "An error occurred. Try again";
      alert(msg);
      setIsLoading(false);
      setIsDisabled(true);
    });
  };

  return (
    <form onSubmit={handleSubmit}>
      <TextField
        label="Vin number"
        required
        value={vin}
        error={!isVinValid}
        onChange={newOnChange(
          /^(?=.*[0-9])(?=.*[A-z])[0-9A-z-]{17}$/,
          setVin,
          setIsVinValid
        )}
        margin="normal"
        sx={{ width: 0.458, mx: 2 }}
      />
      <TextField
        label="Make"
        required
        value={make}
        error={!isMakeValid}
        onChange={newOnChange(/^[a-zA-Z]+$/, setMake, setIsMakeValid)}
        margin="normal"
        sx={{ width: 0.458, mx: 2 }}
      />
      <TextField
        label="Model"
        required
        value={model}
        error={!isModelValid}
        onChange={newOnChange(/^[0-9a-zA-Z\- ]+$/, setModel, setIsModelValid)}
        margin="normal"
        sx={{ width: 0.458, mx: 2 }}
      />
      <TextField
        label="Year"
        required
        value={year}
        error={!isYearValid}
        onChange={newOnChange(
          /^(19[5-9][0-9]|20([01][0-9]|2[0-2]))$/,
          setYear,
          setIsYearValid
        )}
        margin="normal"
        sx={{ width: 0.458, mx: 2 }}
      />
      <Box textAlign="center">
        <LoadingButton
          type="submit"
          variant="text"
          loading={isLoading}
          sx={{ mt: 4 }}
          disabled={
            isDisabled ||
            [isVinValid, isMakeValid, isModelValid, isYearValid].some((v) => !v)
          }
        >
          Confirm
        </LoadingButton>
      </Box>
    </form>
  );
};

export default AddCar;
