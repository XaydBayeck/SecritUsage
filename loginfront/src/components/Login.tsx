import { type Component, createSignal } from 'solid-js';
import { TextInput } from './inputs';

const Login: Component = () => {
  const [user, setUser] = createSignal("")
  const [password, setPassword] = createSignal("")

  const submit = async () => {
    const response = await fetch("/login", {
      method: 'POST',
      body: JSON.stringify({ name: user(), password: password() })
    })
    response.json()
  }

  return (
    <div>
      <TextInput
        name='用户名：'
        placeholder='username/email/phone'
        value={user}
        setValue={setUser} />
      <TextInput
        name='密码：'
        placeholder='password'
        value={password}
        setValue={setPassword} />
      <button onClick={submit}>Login</button>
    </div>
  )
}

export default Login