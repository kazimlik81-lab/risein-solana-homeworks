import * as web3 from "@solana/web3.js";
import { Review } from "@/models/Review";

export const fetchReviews = async (
    REVIEW_PROGRAM_ID: string,
    connection: web3.Connection,
    locationQuery = ""
) => {
    const accounts = await connection.getProgramAccounts(
        new web3.PublicKey(REVIEW_PROGRAM_ID),
        {
            filters: [],
        }
    );
    const query = locationQuery.trim().toLocaleLowerCase();
    return accounts
        .map(({ account }) => Review.deserialize(account.data))
        .filter((review): review is Review => review !== null)
        .filter((review) => !query || review.location.toLocaleLowerCase().includes(query));
};
