import { Link } from "react-router-dom";
import { useState, useEffect } from "react";
import { getAllContacts, deleteContactById } from "../../../api/api";
import {
  Box,
  Typography,
  TableContainer,
  Table,
  TableHead,
  TableBody,
  TableRow,
  TableCell,
} from "@mui/material";
import { Contact } from "models";

const ContactsTable = () => {
  const [contacts, setContacts] = useState<Contact[]>([]);

  useEffect(() => {
    getAllContacts().then((data) => {
      setContacts(data);
    });
  }, []);

  return (
    <Box sx={{ mt: 4, display: "flex", justifyContent: "center" }}>
      <Box sx={{ p: 4, borderRadius: 2, bgcolor: "secondary.main" }}>
        <Typography align="center" variant="h5" style={{ fontWeight: 600 }}>
          Contacts
        </Typography>
        {contacts.length === 0 ? (
          <Typography align="center" variant="h5" sx={{ m: 4 }}>
            An error occurred, try again later
          </Typography>
        ) : (
          <>
            <TableContainer>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Phone number</TableCell>
                    <TableCell>Email</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {contacts.map((c, i) => (
                    <TableRow key={i}>
                      <TableCell>{c.phoneNumber}</TableCell>
                      <TableCell>{c.email}</TableCell>
                      <TableCell>
                        <Link to={"/admin/contacts/edit/" + c.id}>
                          <button>📝</button>
                        </Link>
                      </TableCell>
                      <TableCell>
                        <button
                          onClick={() => {
                            if (
                              confirm(
                                "are you sure you want to delete this contact?"
                              )
                            ) {
                              deleteContactById(c.id);
                              window.location.reload();
                            }
                          }}
                        >
                          ❌
                        </button>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          </>
        )}
      </Box>
    </Box>
  );
};

export default ContactsTable;
