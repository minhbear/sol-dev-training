use anchor_lang::prelude::*;

pub mod state;
pub use state::*;
pub mod contexts;
pub use contexts::*;

declare_id!("7T126mT4jpRayX783R4yEfjXW9yvRMtjYQf9fGWwbDuZ");

#[program]
pub mod anchor_movie_review_program {
    use super::*;

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8
    ) -> Result<()> {
        msg!("Movie Review Account Created");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let moview_review = &mut ctx.accounts.movie_review;
        moview_review.reviewer = ctx.accounts.initializer.key();
        moview_review.title = title;
        moview_review.rating = rating;
        moview_review.description = description;

        Ok(())
    }

    pub fn update_movie_review(
        ctx: Context<UpdateMoviewReview>,
        title: String,
        description: String,
        rating: u8
    ) -> Result<()> {
        msg!("Movie review account space reallocated");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.rating = rating;
        movie_review.description = description;

        Ok(())
    }

    pub fn delete_moview_review(
        ctx: Context<DeleteMovieReview>,
        title: String
    ) -> Result<()> {
        msg!("Movie review for {} deleted", title);
        Ok(())
    }
}


