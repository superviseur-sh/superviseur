import { FC, useState } from "react";
import { Input, SIZE } from "baseui/input";
import styled from "@emotion/styled";
import { SearchOutline } from "@styled-icons/evaicons-outline/SearchOutline";
import { Github } from "@styled-icons/evaicons-solid/Github";
import { Feedback } from "@styled-icons/remix-line/Feedback";

const Container = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  padding: 10px;
  background-color: #630be2;
  height: 40px;
`;

const Logo = styled.div`
  color: #fff;
  font-weight: bold;
  width: 130px;
  text-align: center;
`;

const Settings = styled.div`
  width: 200px;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: flex-end;
  padding-right: 25px;
`;

interface NavbarProps {}

const Navbar: FC<NavbarProps> = () => {
  const [value, setValue] = useState("");
  return (
    <Container>
      <Logo>Superviseur</Logo>
      <Input
        value={value}
        onChange={(e) => setValue(e.target.value)}
        placeholder="Search for a service ..."
        clearOnEscape
        size={SIZE.default}
        startEnhancer={() => <SearchOutline size={20} color="#f9f9f9c6" />}
        overrides={{
          Root: {
            style: {
              width: "400px",
              height: "35px",
              borderWidth: "0px",
              borderRadius: "2px",
              backgroundColor: "#5a10c5",
            },
          },
          Input: {
            style: {
              color: "#fff",
              caretColor: "#fff",
              "::placeholder": {
                color: "#f9f9f990",
              },
              ":-ms-input-placeholder": {
                color: "#f9f9f990",
              },
              "::-ms-input-placeholder": {
                color: "#f9f9f990",
              },
            },
          },
          InputContainer: {
            style: {
              backgroundColor: "#5a10c5",
            },
          },
          StartEnhancer: {
            style: {
              paddingLeft: "0px",
              backgroundColor: "#5a10c5",
            },
          },
        }}
      />
      <Settings>
        <a
          href="https://github.com/tsirysndr/superviseur/issues/new"
          target="_blank"
          rel="noreferrer"
          style={{ marginRight: "20px" }}
          title="Report a bug"
        >
          <Feedback size={24} color="#fff" />
        </a>
        <a
          href="https://github.com/tsirysndr/superviseur"
          target="_blank"
          rel="noreferrer"
          title="View on Github"
        >
          <Github size={24} color="#fff" />
        </a>
      </Settings>
    </Container>
  );
};

export default Navbar;
