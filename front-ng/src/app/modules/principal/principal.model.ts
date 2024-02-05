import { IUser } from "../users";

export interface IPrincipal extends Pick<IUser, 'id' | 'name'> {}
