import {
  getServicesReport,
  getClientsReport,
  getCarsReport,
  getHoursReport,
  getPdfReport,
} from "../../api/api";
import { useState, useEffect } from "react";
import {
  Box,
  Button,
  TableContainer,
  Table,
  TableHead,
  TableBody,
  TableRow,
  TableCell,
  Typography,
} from "@mui/material";

const Reports = () => {
  const [isInitial, setIsInitial] = useState(true);
  const [result, setResult] = useState<any[]>([]);
  const [pdf, setPdf] = useState("");
  useEffect(() => {
    getPdfReport().then((blob) => setPdf(URL.createObjectURL(blob)));
  }, []);

  return (
    <Box
      sx={{ my: 6, mx: 10, p: 4, borderRadius: 4, bgcolor: "secondary.main" }}
    >
      <Box textAlign="center">
        <a href={pdf} target="_blank">
          <Button type="button" variant="text" sx={{ my: 4 }}>
            print pdf report
          </Button>
        </a>
      </Box>
      <Box sx={{ display: "flex", justifyContent: "space-around" }}>
        <Button
          type="button"
          variant="text"
          onClick={() =>
            getServicesReport().then((data) => {
              setResult(data);
              setIsInitial(false);
            })
          }
          sx={{ mb: 4 }}
        >
          services
        </Button>
        <Button
          type="button"
          variant="text"
          onClick={() =>
            getClientsReport().then((data) => {
              setResult(data);
              setIsInitial(false);
            })
          }
          sx={{ mb: 4 }}
        >
          clients
        </Button>
        <Button
          type="button"
          variant="text"
          onClick={() =>
            getCarsReport().then((data) => {
              setResult(data);
              setIsInitial(false);
            })
          }
          sx={{ mb: 4 }}
        >
          cars
        </Button>
        <Button
          type="button"
          variant="text"
          onClick={() =>
            getHoursReport().then((data) => {
              setResult([data]);
              setIsInitial(false);
            })
          }
          sx={{ mb: 4 }}
        >
          working hours
        </Button>
      </Box>
      {result ? (
        <Box textAlign="center" sx={{ mt: 6 }}>
          {result.length ? (
            <TableContainer>
              <Table>
                <TableHead>
                  <TableRow>
                    {Object.keys(result[0]).map((k, i) => (
                      <>
                        {k === "id" ? null : (
                          <TableCell key={i}>
                            {k
                              .replace(
                                /[A-Z]/g,
                                (letter) => ` ${letter.toLowerCase()}`
                              )
                              .replace(
                                /^[a-z]/,
                                (letter) => `${letter.toUpperCase()}`
                              )}
                          </TableCell>
                        )}
                      </>
                    ))}
                  </TableRow>
                </TableHead>
                <TableBody>
                  {result.map((r, i) => (
                    <TableRow key={i}>
                      {Object.keys(r).map((k, i) => (
                        <>
                          {k === "id" ? null : (
                            <TableCell key={i}>{r[k]}</TableCell>
                          )}
                        </>
                      ))}
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          ) : (
            <>
              {isInitial ? (
                <Typography>
                  Press any button above to see selected report
                </Typography>
              ) : (
                <Typography>An error occurred</Typography>
              )}
            </>
          )}
        </Box>
      ) : null}
    </Box>
  );
};

export default Reports;
