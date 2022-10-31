import { useState, useEffect } from "react";
import { getContact } from "../../../api/api";
import EditContactComponent from "../../../components/editContact";
import { Box, Typography } from "@mui/material";
import { Contact } from "models";

const EditContact = () => {
  const [contact, setContact] = useState<Contact>({
    id: "",
    phoneNumber: "",
    email: null,
  });

  useEffect(() => {
    getContact().then((data) => setContact(data));
  }, []);

  return (
    <Box
      sx={{
        display: "flex",
        justifyContent: "center",
      }}
    >
      <Box
        sx={{
          p: 4,
          mt: 4,
          width: 0.6,
          bgcolor: "secondary.main",
          borderRadius: 2,
        }}
      >
        {!contact.id ? (
          <Typography align="center" variant="h5" sx={{ m: 4 }}>
            An error occurred, try again later
          </Typography>
        ) : (
          <EditContactComponent contact={contact} />
        )}
      </Box>
    </Box>
  );
};

export default EditContact;
