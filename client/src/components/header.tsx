import { Link } from "react-router-dom";
import logo from "../assets/logo.png";
import { Box, Typography } from "@mui/material";

const Header = () => {
  return (
    <Box
      sx={{
        px: 4,
        height: "5rem",
        bgcolor: "#2a2836",
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
      }}
    >
      <Link to="/">
        <img src={logo} alt="logo" style={{ width: "3.5rem" }} />
      </Link>
      <Link to="/">
        <Typography
          variant="button"
          sx={{
            fontSize: 16,
            borderRadius: 2,
            color: "secondary.main",
            "&:hover": {
              color: "primary.main",
            },
          }}
        >
          log in
        </Typography>
      </Link>
    </Box>
  );
};

export default Header;
