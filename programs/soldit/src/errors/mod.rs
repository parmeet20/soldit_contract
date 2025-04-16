use anchor_lang::error_code;

#[error_code]
pub enum SolditErrors {
    #[msg("User already initialized.")]
    UserAlreadyInitialized,
    
    #[msg("access denied.")]
    UnauthorizedAccess,

    #[msg("already voted.")]
    AlreadyVoted,

    #[msg("thread not fount.")]
    ThreadNotFound,
    
    #[msg("Title exceeds maximum length of 64 characters.")]
    TitleTooLong,
    
    #[msg("Description exceeds maximum length of 642 characters.")]
    DescriptionTooLong,
    
    #[msg("Image URL exceeds maximum length of 164 characters.")]
    ImageTooLong,
}