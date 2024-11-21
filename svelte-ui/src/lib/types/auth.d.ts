type AuthTokens = {
	refresh?: Token;
	access?: Token;
};
type Token = {
	token: string;
	expiration: number;
};
