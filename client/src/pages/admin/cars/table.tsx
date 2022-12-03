import { Link } from "react-router-dom";
import { useState, useEffect } from "react";
import { getAllCars, deleteCarByVin } from "../../../api/api";
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
import { Car } from "../../../models";

const TableCars = () => {
  const [cars, setCars] = useState<Car[]>([]);

  useEffect(() => {
    getAllCars().then((data) => {
      setCars(data);
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
          Cars
        </Typography>
        {cars.length === 0 ? (
          <Typography align="center" variant="h5" sx={{ m: 4 }}>
            No cars at the moment
          </Typography>
        ) : (
          <>
            <TableContainer>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Vin</TableCell>
                    <TableCell>Make</TableCell>
                    <TableCell>Model</TableCell>
                    <TableCell>Year</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {cars.map((c, i) => (
                    <TableRow key={i}>
                      <TableCell>{c.vin}</TableCell>
                      <TableCell>{c.make}</TableCell>
                      <TableCell>{c.model}</TableCell>
                      <TableCell>{c.year}</TableCell>
                      <TableCell>
                        <Link to={"/admin/cars/edit/" + c.vin}>
                          <button>📝</button>
                        </Link>
                      </TableCell>
                      <TableCell>
                        <button
                          onClick={() => {
                            if (
                              confirm(
                                `Are you sure you want to delete ${c.make} ${c.model} ${c.year}?\n\nNote that all associated orders will be deleted too.`
                              )
                            ) {
                              deleteCarByVin(c.vin);
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
        <Link to="/cars/add">
          <Typography variant="button">{" add new"}</Typography>
        </Link>
      </Typography>
    </Box>
  );
};

export default TableCars;
