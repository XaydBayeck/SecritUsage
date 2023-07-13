interface TextProps {
  name: string,
  placeholder: string,
  value: () => string,
  setValue: (new_value: string) => void
}

export const TextInput = (props: TextProps) => {
  return (
    <div>
      <label>{props.name}</label>
      <input type='text' placeholder={props.placeholder} value={props.value()} onChange={(e) => props.setValue(e.currentTarget.value)} />
    </div>
  )
}
