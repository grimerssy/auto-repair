import React from "react";
import { useState } from "react";
import { Box, TextField } from "@mui/material";
import { LoadingButton } from "@mui/lab";
import { updateServiceById } from "../api/api";
import { Service } from "../models";

type props = {
  service: Service;
};

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

const EditService = ({ service }: props) => {
  const [title, setTitle] = useState(service.title);
  const [isTitleValid, setIsTitleValid] = useState(true);
  const [price, setPrice] = useState(service.price + "");
  const [isPriceValid, setIsPriceValid] = useState(true);
  const [duration, setDuration] = useState(service.duration);
  const [isDurationValid, setIsDurationValid] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [isDisabled, setIsDisabled] = useState(false);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsLoading(true);
    updateServiceById(service.id, {
      title: title,
      price: +price,
      duration: duration,
    }).then((res) => {
      const msg = res.ok
        ? "Updated successfully"
        : "An error occurred. Try again";
      alert(msg);
      setIsLoading(false);
      setIsDisabled(true);
    });
  };

  return (
    <form onSubmit={handleSubmit}>
      <Box sx={{ display: "flex", flexDirection: "column" }}>
        <TextField
          label="Title"
          required
          value={title}
          error={!isTitleValid}
          onChange={newOnChange(/^[a-z]+$/, setTitle, setIsTitleValid)}
          margin="normal"
        />
        <TextField
          label="Price"
          value={price}
          error={!isPriceValid}
          onChange={newOnChange(
            /^[1-9][0-9]+?(\.[0-9]{2})?$/,
            setPrice,
            setIsPriceValid
          )}
          margin="normal"
        />
        <TextField
          label="Duration"
          required
          value={duration}
          error={!isDurationValid}
          onChange={newOnChange(
            /^((1|0?)\d|2[0-3]):([0-5]\d)$/,
            setDuration,
            setIsDurationValid
          )}
          margin="normal"
        />
      </Box>
      <Box textAlign="center">
        <LoadingButton
          type="submit"
          variant="text"
          loading={isLoading}
          sx={{ mt: 4 }}
          disabled={
            isDisabled ||
            [isTitleValid, isPriceValid, isDurationValid].some((v) => !v)
          }
        >
          Confirm
        </LoadingButton>
      </Box>
    </form>
  );
};

export default EditService;
