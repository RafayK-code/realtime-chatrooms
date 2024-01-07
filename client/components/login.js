import { useState } from "react";

async function createAccount({ username, nickname }) {
    try {
        const url = "http://localhost:8080/users/create"; //Not really good practice, url should be an ENV

        let result = await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ username, nickname })
        });
        return result.json();
    }

    catch(e) {
        return Promise.reject(e);
    }
}

async function signIn({ username }) {
    try {
        const url = "http://localhost:8080/users/" + username;
        let result = await fetch(url);
        return result.json();
    }

    catch(e) {
        return Promise.reject(e);
    }
}

export default function Login({ show, setAuth }) {
    const [isShowSignIn, setShowSignIn] = useState(false);

    const showSignIn = () => {
        setShowSignIn(prev => !prev)
    }

    const FormCreateUsername = ({ setAuth }) => {
        const onCreateUsername = async (e) => {
            e.preventDefault();

            let username = e.target.username.value;
            let nickname = e.target.nickname.value;

            if (username === "" || nickname === "") {
                return;
            }
        }
    }
}