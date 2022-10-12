import { createTheme } from "@mui/material/styles";
import { red } from "@mui/material/colors";

const theme = createTheme({
  palette: {
    primary: {
      main: "#E5E7EB",
    },
    secondary: {
      main: "#818CF8",
    },
    error: {
      main: red.A400,
    },
  },
});

export default theme;
