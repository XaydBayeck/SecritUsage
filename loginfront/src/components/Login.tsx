import { type Component, createSignal } from 'solid-js';
import { TextInput } from './inputs';

const Login: Component = () => {
  const [user, setUser] = createSignal("")
  const [password, setPassword] = createSignal("")

  const submit = async () => {
    const email_rp = /^[a-zA-Z0-9_-]+@[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)+$/

    let login = user();

    let body: string

    if (email_rp.test(login)) {
       body = JSON.stringify({ login: { Emial: login }, password: password()})
    } else {
       body = JSON.stringify({ login: { Name: login }, password: password()})
    }

    const response = await fetch("/login", {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: body
    })
    response.json()
  }

  return (
    <div>
      <TextInput
        name='用户名：'
        placeholder='username/email'
        value={user}
        setValue={setUser} />
      <TextInput
        name='密码：'
        type='password'
        placeholder='password'
        setValue={setPassword} />
      <button onClick={submit}>Login</button>
      <a href='/signup'>注册</a>
    </div>
  )
}

export default Login