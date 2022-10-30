import { Link } from "react-router-dom";
import { useState, useEffect } from "react";
import { getAllServices, deleteServiceById } from "../../../api/api";
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
import { Service } from "models";

const OrdersByService = () => {
  const [services, setServices] = useState<Service[]>([]);

  useEffect(() => {
    getAllServices().then((data) => {
      setServices(data);
    });
  }, []);

  return (
    <Box
      sx={{
        mt: 4,
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Box
        sx={{ p: 4, borderRadius: 2, width: 0.6, bgcolor: "secondary.main" }}
      >
        <Typography align="center" variant="h5" style={{ fontWeight: 600 }}>
          Services
        </Typography>
        {services.length === 0 ? (
          <Typography align="center" variant="h5" sx={{ m: 4 }}>
            No available services at the moment
          </Typography>
        ) : (
          <>
            <TableContainer>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Title</TableCell>
                    <TableCell>Price</TableCell>
                    <TableCell>Duration</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {services.map((s, i) => (
                    <TableRow key={i}>
                      <TableCell>{s.title}</TableCell>
                      <TableCell>{s.price}</TableCell>
                      <TableCell>{s.duration}</TableCell>
                      <TableCell>
                        <Link to={"/admin/services/edit/" + s.id}>
                          <button>📝</button>
                        </Link>
                      </TableCell>
                      <TableCell>
                        <button
                          onClick={() => {
                            if (
                              confirm(
                                "Are you sure you want to delete this service?"
                              )
                            ) {
                              deleteServiceById(s.id);
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
      <Typography
        variant="overline"
        align="center"
        sx={{ bgcolor: "secondary.main", p: 4, borderRadius: 4, m: 4 }}
      >
        or you can
        <Link to="/admin/services/add">
          <Typography variant="button">{" add new"}</Typography>
        </Link>
      </Typography>
    </Box>
  );
};

export default OrdersByService;
