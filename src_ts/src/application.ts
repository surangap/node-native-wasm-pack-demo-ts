import {sayHelloWithObject, sayHelloWithString, Resp} from '../../pkg/hello_wasm'

{
  // say hello with string
  const response = sayHelloWithString('Guest')
  console.log(response);
}

{
  // say hello with object
  const resp = sayHelloWithObject('Guest2')
  console.log(resp);
  console.log(resp.getString())

  // set value from ts
  resp.setString("Guest3")
  console.log(resp.getString())
}

{
  // create the object in ts and manipulate
  const resp = Resp.new("Guest4")
  console.log(resp.getString())
  
  resp.setString("Guest5")
  console.log(resp.getString())
}

