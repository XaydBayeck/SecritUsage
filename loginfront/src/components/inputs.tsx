interface TextProps {
  name: string,
  placeholder: string,
  value?: () => string,
  setValue: (new_value: string) => void,
  type?: 'text' | 'password',
}

export const TextInput = (props: TextProps) => {
  return (
    <div>
      <label>{props.name}</label>
      <input
        type={props.type ? props.type : 'text'}
        placeholder={props.placeholder}
        value={props.value ? props.value() : ""}
        onChange={(e) => props.setValue(e.currentTarget.value)} />
    </div>
  )
}
