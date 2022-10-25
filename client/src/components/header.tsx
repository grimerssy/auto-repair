import { Link } from "react-router-dom";
import logo from "../assets/logo.png";
import { Box, Typography } from "@mui/material";
import jwt from "jwt-decode";

type Claims = {
  sub: string;
  role: string;
};

const Header = () => {
  const token = localStorage.getItem("accessToken");
  let msg: string | null = "log in";
  let linkTo: string = "/auth/login";
  try {
    const claims: Claims = jwt(token || "");
    switch (claims.role) {
      case "user":
        [msg, linkTo] = [null, "/"];
      case "admin":
        [msg, linkTo] = ["admin panel", "/admin"];
    }
  } catch { }

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
      <Link to={linkTo}>
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
          {msg}
        </Typography>
      </Link>
    </Box>
  );
};

export default Header;
