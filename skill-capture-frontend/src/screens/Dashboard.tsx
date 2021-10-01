import React from "react";
import ReactDOM from "react-dom";
import { Flex, Box } from "rebass";
import {
  TextInputField,
  Text,
  Pane,
  Heading,
  Button,
  Link
} from "evergreen-ui";
import {
  Link as RouteLink
} from "react-router-dom";

export function Dashboard() {
  return (
    <div>
      <Flex justifyContent="center">
        <Box width={[1, 1 / 2, 1 / 3]}>
          <Pane
            background="tint2"
            display="flex"
            flexWrap="wrap"
            justifyContent="center"
            flexDirection="column"
            padding={20}
          >
            <Heading size={700} textAlign="center" paddingBottom="2rem">
              Welcome to Track Me
            </Heading>
            {/* <TextInputField required label="Username" />
            <TextInputField required type="password" label="Password" /> */}
            <Flex>
              <Button appearance="primary" justifyContent="center">
                <RouteLink to="/score">Score your collegue</RouteLink>
              </Button>
              <Button justifyContent="center">
              <RouteLink to="/query">Query</RouteLink>
              </Button>
            </Flex>
            {/* <Text textAlign="center" marginTop="2rem">
              Forgot your password? <Link href="#"> Reset your password</Link>
            </Text>
            <Text textAlign="center">
              Don't have an account? <Link href="#">Sign up</Link>
            </Text> */}
          </Pane>
        </Box>
      </Flex>
    </div>
  );
}

