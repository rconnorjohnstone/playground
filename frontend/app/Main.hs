{-# LANGUAGE OverloadedStrings #-}

import Network.Wai
import Network.Wai.Handler.Warp                 (run)
import Network.Wai.Middleware.Gzip              (gzip, def)
import Network.Wai.Middleware.RequestLogger     (logStdout)
import Network.Wai.Application.Static
import Network.HTTP.Types                       (status200)
import Blaze.ByteString.Builder                 (fromByteString)
import Blaze.ByteString.Builder.Char.Utf8       (fromShow)

application req respond = do
  respond $ responseBuilder
    status200 
    [("Content-Type", "text/plain")] $
    case lookup "host" $ requestHeaders req of
      Just x -> fromByteString "i hope so"
      Nothing -> fromByteString "Whoops"

main = do
  let port = 3000
  putStrLn ""
  putStrLn "Spinning up the awesome server"
  run port $ gzip def $ logStdout application
