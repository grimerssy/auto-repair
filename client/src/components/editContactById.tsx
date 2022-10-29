import React from "react";
import { useState } from "react";
import { Box, TextField } from "@mui/material";
import { LoadingButton } from "@mui/lab";
import { updateContactById } from "../api/api";
import { Contact } from "../models";

type props = {
  id: string;
  contact: Contact;
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

const EditContact = ({ id, contact }: props) => {
  const [phoneNumber, setPhoneNumber] = useState(contact.phoneNumber);
  const [isPhoneNumberValid, setIsPhoneNumberValid] = useState(true);
  const [email, setEmail] = useState(contact.email || "");
  const [isEmailValid, setIsEmailValid] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [isDisabled, setIsDisabled] = useState(false);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsLoading(true);
    updateContactById(id, {
      phoneNumber: phoneNumber,
      email: email || null,
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
          label="Phone number"
          required
          value={phoneNumber}
          error={!isPhoneNumberValid}
          onChange={newOnChange(
            /^[0-9]{10}$/,
            setPhoneNumber,
            setIsPhoneNumberValid
          )}
          margin="normal"
        />
        <TextField
          label="Email"
          value={email}
          error={!isEmailValid}
          onChange={newOnChange(
            /^(|[a-zA-Z]+@[a-zA-Z]+\.[a-zA-Z]+)$/,
            setEmail,
            setIsEmailValid
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
            isDisabled || [isPhoneNumberValid, isEmailValid].some((v) => !v)
          }
        >
          Confirm
        </LoadingButton>
      </Box>
    </form>
  );
};

export default EditContact;
