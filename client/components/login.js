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

            let res = await createAccount({ username, nickname });
            if (res === null) {
                alert("Failed to create account");
                return;
            }

            setAuth(res);
        }

        return (
            <form action="" className="mt-4 space-y-2" onSubmit={onCreateUsername}>
                <div>
                    <label className="text-sm font-light">Username</label>
                    <input required type="text" name="username" placeholder="John Doe"
                        className="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600" />
                </div>
                <div>
                    <label className="text-sm font-light">Nickname</label>
                    <input required type="text" name="phone" placeholder="+1111..."
                        className="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600" />
                </div>
                <div className="flex items-baseline justify-between">
                    <button type="submit"
                        className="px-6 py-2 mt-4 text-white bg-violet-600 rounded-lg hover:bg-violet-700 w-full">Submit</button>
                </div>
                <div className="pt-2 space-y-2 text-center">
                    <p className="text-base text-gray-700">Already have a username? <button onClick={showSignIn} className="text-violet-700 font-light">Sign In</button></p>
                </div>
            </form>
        )
    }

    const FormSignIn = ({ setAuth }) => {
        const onSignIn = async (e) => {
            e.preventDefault();
            let username = e.target.username;

            if (phone === "") {
                return;
            }

            let res = await signIn({ username });
            if (res === null) {
                alert("Failed to sign in");
                return;
            }

            if (!res.id) {
                alert("Username not found");
                return;
            }

            setAuth(res)
        }

        return (
            <form action="" className="mt-4 space-y-2" onSubmit={onSignIn}>
                <div>
                    <label className="text-sm font-light">Phone</label>
                    <input required type="text" name="phone" placeholder="+1111..."
                        className="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600" />
                </div>
                <div className="flex items-baseline justify-between">
                    <button type="submit"
                        className="px-6 py-2 mt-4 text-white bg-violet-600 rounded-lg hover:bg-violet-700 w-full">Submit</button>
                </div>
                <div className="pt-2 space-y-2 text-center">
                    <p className="text-base text-gray-700">Don't have username? <button onClick={showSignIn} className="text-violet-700 font-light">Create</button></p>
                </div>
            </form>
        )
    }

    return (
        <div className={`${show ? '' : 'hidden'} bg-gradient-to-b from-orange-400 to-rose-400`}>
            <div className="flex items-center justify-center min-h-screen">
                <div className="px-8 py-6 mt-4 text-left bg-white  max-w-[400px] w-full rounded-xl shadow-lg">
                    <h3 className="text-xl text-slate-800 font-semibold">{isShowSignIn ? 'Log in with your phone.' : 'Create your account.'}</h3>
                    {isShowSignIn ? <FormSignIn setAuth={setAuth} /> : <FormCreateUsername setAuth={setAuth} />}
                </div>
            </div>
        </div>
    )
}