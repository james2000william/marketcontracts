use cosmwasm_std::{
    Api, CanonicalAddr, Extern, HumanAddr, Querier, StdError, StdResult, Storage, Uint128,
};

pub type Tokens = Vec<(CanonicalAddr, Uint128)>; // <(Collateral Token, Amount)>
pub type TokensHuman = Vec<(HumanAddr, Uint128)>;

pub trait TokensMath {
    fn sub(self: &mut Self, collaterals: Tokens) -> StdResult<()>;
    fn add(self: &mut Self, collaterals: Tokens);
}

pub trait TokensToHuman {
    fn to_human<S: Storage, A: Api, Q: Querier>(
        self: &Self,
        deps: &Extern<S, A, Q>,
    ) -> StdResult<TokensHuman>;
}

pub trait TokensToRaw {
    fn to_raw<S: Storage, A: Api, Q: Querier>(
        self: &Self,
        deps: &Extern<S, A, Q>,
    ) -> StdResult<Tokens>;
}

impl TokensMath for Tokens {
    fn sub(self: &mut Self, tokens: Tokens) -> StdResult<()> {
        self.sort_by(|a, b| a.0.as_slice().cmp(&b.0.as_slice()));

        let mut tokens = tokens.clone();
        tokens.sort_by(|a, b| a.0.as_slice().cmp(&b.0.as_slice()));

        let mut i = 0;
        let mut j = 0;
        while i < self.len() && j < tokens.len() {
            if self[i].0 == tokens[j].0 {
                self[i].1 = (self[i].1 - tokens[j].1)?;

                i += 1;
                j += 1;
            } else if self[i].0.as_slice().cmp(&tokens[j].0.as_slice())
                == std::cmp::Ordering::Greater
            {
                j += 1;
            } else {
                i += 1;
            }
        }

        if j != tokens.len() {
            return Err(StdError::generic_err("Subtraction underflow"));
        }

        Ok(())
    }

    fn add(self: &mut Self, tokens: Tokens) {
        self.sort_by(|a, b| a.0.as_slice().cmp(&b.0.as_slice()));

        let mut tokens = tokens.clone();
        tokens.sort_by(|a, b| a.0.as_slice().cmp(&b.0.as_slice()));

        let mut i = 0;
        let mut j = 0;
        while i < self.len() && j < tokens.len() {
            if self[i].0 == tokens[j].0 {
                self[i].1 += tokens[j].1;

                i += 1;
                j += 1;
            } else if self[i].0.as_slice().cmp(&tokens[j].0.as_slice())
                == std::cmp::Ordering::Greater
            {
                j += 1;
            } else {
                i += 1;
            }
        }

        while j < tokens.len() {
            self.push(tokens[j].clone());
            j += 1;
        }
    }
}

impl TokensToHuman for Tokens {
    fn to_human<S: Storage, A: Api, Q: Querier>(
        self: &Self,
        deps: &Extern<S, A, Q>,
    ) -> StdResult<TokensHuman> {
        let collaterals: TokensHuman = self
            .iter()
            .map(|c| Ok((deps.api.human_address(&c.0)?, c.1)))
            .collect::<StdResult<TokensHuman>>()?;
        Ok(collaterals)
    }
}

impl TokensToRaw for TokensHuman {
    fn to_raw<S: Storage, A: Api, Q: Querier>(
        self: &Self,
        deps: &Extern<S, A, Q>,
    ) -> StdResult<Tokens> {
        let collaterals: Tokens = self
            .iter()
            .map(|c| Ok((deps.api.canonical_address(&c.0)?, c.1)))
            .collect::<StdResult<Tokens>>()?;
        Ok(collaterals)
    }
}