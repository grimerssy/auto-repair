import { useParams } from "react-router-dom";
import { useState, useEffect } from "react";
import { getContactById } from "../../../api/api";
import EditContactComponent from "../../../components/editContactById";
import { Box, Typography } from "@mui/material";
import { Contact } from "models";

const EditContact = () => {
  const { id } = useParams();

  const [contact, setContact] = useState<Contact>({
    id: "",
    phoneNumber: "",
    email: null,
  });

  useEffect(() => {
    getContactById(id || "").then((data) => setContact(data));
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
          <EditContactComponent id={id || ""} contact={contact} />
        )}
      </Box>
    </Box>
  );
};

export default EditContact;
