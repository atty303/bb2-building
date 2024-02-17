import {createAuth0Client} from "@auth0/auth0-spa-js";

export async function createAuth0() {
    return await createAuth0Client({
        domain: "bb2b.us.auth0.com",
        clientId: "udNY8zDu6nALh3lQFJaYykONTiJgGob1",
    });
}
