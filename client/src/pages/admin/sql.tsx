import { executeSql } from "../../api/api";
import { useState } from "react";
import { LoadingButton } from "@mui/lab";
import {
  Box,
  TextField,
  TableContainer,
  Table,
  TableHead,
  TableBody,
  TableRow,
  TableCell,
  Typography,
} from "@mui/material";

const SqlEdit = () => {
  const [query, setQuery] = useState("");
  const [result, setResult] = useState<string | any[] | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsLoading(true);
    executeSql(query).then((res) => {
      console.log(res);
      setResult(res);
      setIsLoading(false);
    });
  };

  return (
    <Box
      sx={{ my: 6, mx: 10, p: 4, borderRadius: 4, bgcolor: "secondary.main" }}
    >
      <form onSubmit={handleSubmit}>
        <TextField
          label="Query"
          required
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          fullWidth
          multiline
          rows={4}
        />
        <Box textAlign="center">
          <LoadingButton
            type="submit"
            variant="text"
            loading={isLoading}
            sx={{ mt: 4 }}
          >
            execute
          </LoadingButton>
        </Box>
      </form>
      {result ? (
        <Box textAlign="center" sx={{ mt: 6 }}>
          {typeof result === "string" ? (
            <Typography>{result}</Typography>
          ) : (
            <>
              {result.length ? (
                <TableContainer>
                  <Table>
                    <TableHead>
                      <TableRow>
                        {Object.keys(result[0]).map((k, i) => (
                          <TableCell key={i}>{k}</TableCell>
                        ))}
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {result.map((r, i) => (
                        <TableRow key={i}>
                          {Object.keys(r).map((k, i) => (
                            <TableCell key={i}>{r[k]}</TableCell>
                          ))}
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </TableContainer>
              ) : (
                <Typography>No results were found</Typography>
              )}
            </>
          )}
        </Box>
      ) : null}
    </Box>
  );
};

export default SqlEdit;
